import Server from './server'
import db from './database'
import config from '../config'
import * as extensions from './extensions'

export const server = new Server({
	port: config.port,
	limits: {
		global: '20/5s',
		'auth/login': '3/24h --ip',
		'auth/register': '3/24h --ip',
		'auth/verify': '2/24h --ip',
		servers: '5/5s',
		channels: '5/5s',
		users: '5/5s'
	},
	extensions: (req, res) => {
		extensions.check(req, res)
	}
});

(async () => {
	console.log('Initialling the server...')

	await server.init()

	console.log('Connecting to database...')

	await db.connect()

	console.log('Connected to Database')

	await server.listen()

	console.log(`Server running on port: ${config.port}`)
})().catch(err => {
	console.error('Failed to init the server...', err)
	console.error('Exiting..')
	process.exit(-1)
})


process
	.on('unhandledRejection', err => console.error(err))
	.on('uncaughtException', console.error)