import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { Message as T } from '../structures'
import { getaway } from '../server'

@Subscriber()
export class MessageSubscriber implements EventSubscriber<T> {
	async afterCreate({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channel._id, 'MESSAGE_CREATE', message)
	}

	async afterUpdate({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channel._id, 'MESSAGE_UPDATE', message)
	}

	async afterDelete({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channel._id, 'MESSAGE_DELETE', {
			_id: message._id,
			channel: message.channel
		})
	}
}