import { Definition } from '@/routes/definitions/$definitionId/-feature/components/Definition'
import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/definitions/$definitionId/')({
  component: RouteComponent,
})

function RouteComponent() {
  const {definitionId} = Route.useParams()
  return <Definition definitionId={definitionId}/>
}
