targetScope = 'resourceGroup'

@description('Azure region used for all resources.')
param location string = resourceGroup().location

@description('Unique azd environment name used to derive resource names.')
@minLength(1)
@maxLength(32)
param environmentName string

@description('Azure Log Analytics workspace name. Defaults to a name derived from the environment.')
param logAnalyticsWorkspaceName string = ''

@description('Azure Container Apps environment name. Defaults to a name derived from the environment.')
param containerAppsEnvironmentName string = ''

@description('Azure Container App name for the MCP service. Defaults to a name derived from the environment.')
param containerAppName string = ''

@secure()
@description('Shared secret used by the service for Bearer authentication in v1.')
@minLength(16)
param mcpApiKey string

@description('Image reference for the MCP service container. Updated by azd during deployment.')
param containerImage string = 'mcr.microsoft.com/azuredocs/containerapps-helloworld:latest'

@description('Number of CPU cores allocated to each replica.')
param containerCpu string = '0.5'

@description('Memory allocated to each replica.')
param containerMemory string = '1.0Gi'

@description('Minimum replica count. Set to 1 to disable cold starts.')
@minValue(0)
@maxValue(10)
param minReplicas int = 0

@description('Maximum replica count for HTTP-driven autoscale.')
@minValue(1)
@maxValue(30)
param maxReplicas int = 3

@description('Concurrent HTTP requests per replica before the scaler adds capacity.')
@minValue(1)
@maxValue(1000)
param scaleConcurrentRequests int = 50

@description('Enable HTTP liveness/readiness probes against /healthz. Keep false for the first deployment (placeholder image does not implement /healthz); set to true once the real image is deployed via `azd deploy`.')
param enableProbes bool = false

@description('Azure Container Registry name. Defaults to a name derived from the environment.')
param containerRegistryName string = ''

var containerAppPort = 8080
var resourceToken = toLower(uniqueString(subscription().id, resourceGroup().id, environmentName))
var resolvedLogAnalyticsWorkspaceName = empty(logAnalyticsWorkspaceName) ? 'log-${environmentName}-${resourceToken}' : logAnalyticsWorkspaceName
var resolvedContainerAppsEnvironmentName = empty(containerAppsEnvironmentName) ? 'cae-${environmentName}-${resourceToken}' : containerAppsEnvironmentName
var resolvedContainerAppName = empty(containerAppName) ? 'ca-${environmentName}-${resourceToken}' : containerAppName
var resolvedContainerRegistryName = empty(containerRegistryName) ? 'cr${replace(environmentName, '-', '')}${resourceToken}' : containerRegistryName
var acrPullRoleId = subscriptionResourceId('Microsoft.Authorization/roleDefinitions', '7f951dda-4ed3-4680-a7ca-43fe172d538d')

var probesConfig = [
  {
    type: 'Liveness'
    httpGet: {
      path: '/healthz'
      port: containerAppPort
    }
    initialDelaySeconds: 5
    periodSeconds: 30
    failureThreshold: 3
  }
  {
    type: 'Readiness'
    httpGet: {
      path: '/healthz'
      port: containerAppPort
    }
    initialDelaySeconds: 2
    periodSeconds: 10
    failureThreshold: 3
  }
]

var commonTags = {
  'azd-env-name': environmentName
  project: 'rust-mcp-azure'
  'managed-by': 'bicep'
}

resource logAnalyticsWorkspace 'Microsoft.OperationalInsights/workspaces@2023-09-01' = {
  name: resolvedLogAnalyticsWorkspaceName
  location: location
  tags: commonTags
  properties: {
    sku: {
      name: 'PerGB2018'
    }
    retentionInDays: 30
  }
}

resource containerRegistry 'Microsoft.ContainerRegistry/registries@2023-07-01' = {
  name: resolvedContainerRegistryName
  location: location
  tags: commonTags
  sku: {
    name: 'Basic'
  }
  properties: {
    adminUserEnabled: false
    publicNetworkAccess: 'Enabled'
  }
}

resource acrPullRoleAssignment 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(containerRegistry.id, containerApp.id, acrPullRoleId)
  scope: containerRegistry
  properties: {
    principalId: containerApp.identity.principalId
    roleDefinitionId: acrPullRoleId
    principalType: 'ServicePrincipal'
  }
}

resource containerAppsEnvironment 'Microsoft.App/managedEnvironments@2024-03-01' = {
  name: resolvedContainerAppsEnvironmentName
  location: location
  tags: commonTags
  properties: {
    appLogsConfiguration: {
      destination: 'log-analytics'
      logAnalyticsConfiguration: {
        customerId: logAnalyticsWorkspace.properties.customerId
        sharedKey: listKeys(logAnalyticsWorkspace.id, logAnalyticsWorkspace.apiVersion).primarySharedKey
      }
    }
  }
}

resource containerApp 'Microsoft.App/containerApps@2024-03-01' = {
  name: resolvedContainerAppName
  location: location
  tags: union(commonTags, {
    'azd-service-name': 'api'
  })
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    managedEnvironmentId: containerAppsEnvironment.id
    configuration: {
      activeRevisionsMode: 'Single'
      ingress: {
        allowInsecure: false
        external: true
        targetPort: containerAppPort
        transport: 'http'
      }
      secrets: [
        {
          name: 'mcp-api-key'
          value: mcpApiKey
        }
      ]
      registries: [
        {
          server: containerRegistry.properties.loginServer
          identity: 'system'
        }
      ]
    }
    template: {
      containers: [
        {
          name: 'api'
          image: containerImage
          env: [
            {
              name: 'PORT'
              value: string(containerAppPort)
            }
            {
              name: 'RUST_LOG'
              value: 'info'
            }
            {
              name: 'MCP_SANDBOX_ROOT'
              value: '/app'
            }
            {
              name: 'MCP_ENABLE_PROCESS_SUMMARY'
              value: 'false'
            }
            {
              name: 'MCP_API_KEY'
              secretRef: 'mcp-api-key'
            }
          ]
          resources: {
            cpu: json(containerCpu)
            memory: containerMemory
          }
          probes: enableProbes ? probesConfig : []
        }
      ]
      scale: {
        minReplicas: minReplicas
        maxReplicas: maxReplicas
        rules: [
          {
            name: 'http-concurrency'
            http: {
              metadata: {
                concurrentRequests: string(scaleConcurrentRequests)
              }
            }
          }
        ]
      }
    }
  }
}

output SERVICE_API_ENDPOINT_URL string = 'https://${containerApp.properties.configuration.ingress.fqdn}'
output SERVICE_API_RESOURCE_NAME string = containerApp.name
output SERVICE_API_IDENTITY_PRINCIPAL_ID string = containerApp.identity.principalId
output AZURE_CONTAINER_APPS_ENVIRONMENT_NAME string = containerAppsEnvironment.name
output AZURE_LOG_ANALYTICS_WORKSPACE_NAME string = logAnalyticsWorkspace.name
output AZURE_LOG_ANALYTICS_WORKSPACE_ID string = logAnalyticsWorkspace.id
output AZURE_CONTAINER_REGISTRY_ENDPOINT string = containerRegistry.properties.loginServer
output AZURE_CONTAINER_REGISTRY_NAME string = containerRegistry.name
