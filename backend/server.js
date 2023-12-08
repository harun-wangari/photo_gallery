const express = require('express')
const bodyParser = require('body-parser')
const cors = require('cors')

const app = express()
const port = 3000

app.use(cors())
app.use(bodyParser.json())

app.get('/',(req,res) => {
    res.send("server is running")
})

app.post('/login',(req,res) => {
    console.log(req.body)
    res.send(req.body)
})

app.listen(port, () => {
    console.log("online")
})