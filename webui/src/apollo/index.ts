import type { ApolloClientOptions } from '@apollo/client/core'
import { createHttpLink, InMemoryCache, split } from '@apollo/client/core'
// import type { BootFileParams } from '@quasar/app'
import { GraphQLWsLink } from '@apollo/client/link/subscriptions'
import { createClient } from 'graphql-ws'
import { getMainDefinition } from '@apollo/client/utilities'
import * as constants from 'src/const'
import { Cookies } from 'quasar'

export /* async */ function getClientOptions (/* {app, router, ...}, options?: Partial<BootFileParams<unknown>> */) {
  const port = Cookies.get('service-port') || constants.port
  const host = Cookies.get('service-host') || constants.host
  const wsLink = new GraphQLWsLink(
    createClient({
      url: 'ws://' + host + ':' + port + '/ws'
    })
  )

  const httpLink = createHttpLink({
    uri: (operation) => {
      const chainId = operation.variables.chainId as string
      switch (operation.variables.endpoint) {
        case 'feed':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.feedApp
        case 'credit':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.creditApp
        case 'market':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.marketApp
        case 'review':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.reviewApp
        case 'foundation':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.foundationApp
        case 'activity':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.activityApp
        case 'copilot':
        {
          let applicationId = constants.Apps.copilotApp
          if (operation.variables.applicationId) {
            // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
            applicationId = operation.variables.applicationId
          }
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + applicationId
        }
        case 'cp-registry':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.cpRegistryApp
        case 'blob-gateway':
          return 'http://' + host + ':' + port + '/chains/' + chainId + '/applications/' + constants.Apps.blobGatewayApp
        case 'main':
          return 'http://' + host + ':' + port
        default:
          return 'http://' + host + ':' + port
      }
    }
  })

  const splitLink = split(
    ({ query }) => {
      const definition = getMainDefinition(query)
      return (
        definition.kind === 'OperationDefinition' &&
        definition.operation === 'subscription'
      )
    },
    wsLink,
    httpLink
  )

  return <ApolloClientOptions<unknown>>Object.assign(
    // General options.
    <ApolloClientOptions<unknown>>{
      link: splitLink,
      cache: new InMemoryCache()
    },

    // Specific Quasar mode options.
    process.env.MODE === 'spa'
      ? {
          //
        }
      : {},
    process.env.MODE === 'ssr'
      ? {
          //
        }
      : {},
    process.env.MODE === 'pwa'
      ? {
          //
        }
      : {},
    process.env.MODE === 'bex'
      ? {
          //
        }
      : {},
    process.env.MODE === 'cordova'
      ? {
          //
        }
      : {},
    process.env.MODE === 'capacitor'
      ? {
          //
        }
      : {},
    process.env.MODE === 'electron'
      ? {
          //
        }
      : {},

    // dev/prod options.
    process.env.DEV
      ? {
          //
        }
      : {},
    process.env.PROD
      ? {
          //
        }
      : {},

    // For ssr mode, when on server.
    process.env.MODE === 'ssr' && process.env.SERVER
      ? {
          ssrMode: true
        }
      : {},
    // For ssr mode, when on client.
    process.env.MODE === 'ssr' && process.env.CLIENT
      ? {
          ssrForceFetchDelay: 100
        }
      : {}
  )
}
