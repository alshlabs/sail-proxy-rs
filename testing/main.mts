import {chat} from './chat.mts'

// @ts-expect-error
const TOKEN = process.env.SAIL_API_KEY

console.log(await chat(
    'http://localhost:5051/v1', 
    TOKEN,
    "deepseek-flash",
    'Hellooo'
))
