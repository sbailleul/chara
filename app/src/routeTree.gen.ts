/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'

// Create Virtual Routes

const IndexLazyImport = createFileRoute('/')()
const GraphIndexLazyImport = createFileRoute('/graph/')()
const DefinitionsIndexLazyImport = createFileRoute('/definitions/')()
const DefinitionsDefinitionIdIndexLazyImport = createFileRoute(
  '/definitions/$definitionId/',
)()

// Create/Update Routes

const IndexLazyRoute = IndexLazyImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/index.lazy').then((d) => d.Route))

const GraphIndexLazyRoute = GraphIndexLazyImport.update({
  id: '/graph/',
  path: '/graph/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/graph/index.lazy').then((d) => d.Route))

const DefinitionsIndexLazyRoute = DefinitionsIndexLazyImport.update({
  id: '/definitions/',
  path: '/definitions/',
  getParentRoute: () => rootRoute,
} as any).lazy(() =>
  import('./routes/definitions/index.lazy').then((d) => d.Route),
)

const DefinitionsDefinitionIdIndexLazyRoute =
  DefinitionsDefinitionIdIndexLazyImport.update({
    id: '/definitions/$definitionId/',
    path: '/definitions/$definitionId/',
    getParentRoute: () => rootRoute,
  } as any).lazy(() =>
    import('./routes/definitions/$definitionId/index.lazy').then(
      (d) => d.Route,
    ),
  )

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/definitions/': {
      id: '/definitions/'
      path: '/definitions'
      fullPath: '/definitions'
      preLoaderRoute: typeof DefinitionsIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/graph/': {
      id: '/graph/'
      path: '/graph'
      fullPath: '/graph'
      preLoaderRoute: typeof GraphIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/definitions/$definitionId/': {
      id: '/definitions/$definitionId/'
      path: '/definitions/$definitionId'
      fullPath: '/definitions/$definitionId'
      preLoaderRoute: typeof DefinitionsDefinitionIdIndexLazyImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  '/': typeof IndexLazyRoute
  '/definitions': typeof DefinitionsIndexLazyRoute
  '/graph': typeof GraphIndexLazyRoute
  '/definitions/$definitionId': typeof DefinitionsDefinitionIdIndexLazyRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexLazyRoute
  '/definitions': typeof DefinitionsIndexLazyRoute
  '/graph': typeof GraphIndexLazyRoute
  '/definitions/$definitionId': typeof DefinitionsDefinitionIdIndexLazyRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexLazyRoute
  '/definitions/': typeof DefinitionsIndexLazyRoute
  '/graph/': typeof GraphIndexLazyRoute
  '/definitions/$definitionId/': typeof DefinitionsDefinitionIdIndexLazyRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths: '/' | '/definitions' | '/graph' | '/definitions/$definitionId'
  fileRoutesByTo: FileRoutesByTo
  to: '/' | '/definitions' | '/graph' | '/definitions/$definitionId'
  id:
    | '__root__'
    | '/'
    | '/definitions/'
    | '/graph/'
    | '/definitions/$definitionId/'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexLazyRoute: typeof IndexLazyRoute
  DefinitionsIndexLazyRoute: typeof DefinitionsIndexLazyRoute
  GraphIndexLazyRoute: typeof GraphIndexLazyRoute
  DefinitionsDefinitionIdIndexLazyRoute: typeof DefinitionsDefinitionIdIndexLazyRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexLazyRoute: IndexLazyRoute,
  DefinitionsIndexLazyRoute: DefinitionsIndexLazyRoute,
  GraphIndexLazyRoute: GraphIndexLazyRoute,
  DefinitionsDefinitionIdIndexLazyRoute: DefinitionsDefinitionIdIndexLazyRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/definitions/",
        "/graph/",
        "/definitions/$definitionId/"
      ]
    },
    "/": {
      "filePath": "index.lazy.tsx"
    },
    "/definitions/": {
      "filePath": "definitions/index.lazy.tsx"
    },
    "/graph/": {
      "filePath": "graph/index.lazy.tsx"
    },
    "/definitions/$definitionId/": {
      "filePath": "definitions/$definitionId/index.lazy.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
