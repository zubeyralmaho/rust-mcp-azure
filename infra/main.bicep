targetScope = 'resourceGroup'

@description('Azure region used for all resources.')
param location string = resourceGroup().location

@description('Unique azd environment name used to derive resource names.')
param environmentName string

@description('Azure Log Analytics workspace name.')
param logAnalyticsWorkspaceName string = 'log-${environmentName}'

@description('Azure Container Apps environment name.')
param containerAppsEnvironmentName string = 'cae-${environmentName}'

@description('Azure Container App name for the MCP service.')
param containerAppName string = 'ca-${environmentName}'

@secure()
@description('Shared secret used by the service for Bearer authentication in v1.')
param mcpApiKey string

var containerAppPort = 8080
var placeholderImage = 'mcr.microsoft.com/azuredocs/containerapps-helloworld:latest'

resource logAnalyticsWorkspace 'Microsoft.OperationalInsights/workspaces@2023-09-01' = {
  name: logAnalyticsWorkspaceName
  location: location
  properties: {
    sku: {
      name: 'PerGB2018'
    }
    retentionInDays: 30
  }
}

resource containerAppsEnvironment 'Microsoft.App/managedEnvironments@2024-03-01' = {
  name: containerAppsEnvironmentName
  location: location
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
  name: containerAppName
  location: location
  tags: {
    'azd-service-name': 'api'
    'azd-env-name': environmentName
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
    }
    template: {
      containers: [
        {
          name: 'api'
          image: placeholderImage
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
            cpu: 0.5
            memory: '1Gi'
          }
        }
      ]
      scale: {
        minReplicas: 0
        maxReplicas: 1
      }
    }
  }
}

output SERVICE_API_ENDPOINT_URL string = 'https://${containerApp.properties.configuration.ingress.fqdn}'
output SERVICE_API_RESOURCE_NAME string = containerApp.name
output AZURE_CONTAINER_APPS_ENVIRONMENT_NAME string = containerAppsEnvironment.name
output AZURE_LOG_ANALYTICS_WORKSPACE_NAME string = logAnalyticsWorkspace.name