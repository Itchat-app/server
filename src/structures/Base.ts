import { PrimaryKey } from 'mikro-orm'
import { Snowflake } from '../utils'

export abstract class Base {
  @PrimaryKey({ unique: true })
  _id!: ID

  setID(): this {
    this._id = Snowflake.generate()
    return this
  }
}
