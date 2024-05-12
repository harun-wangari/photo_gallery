const express = require('express')
const bodyParser = require('body-parser')
const cors = require('cors')

const app = express()
const port = 3000

app.use(cors())
app.use(bodyParser.json())

const users = require("./user.json")

app.get('/',(req,res) => {
    res.send("server is running")
})

app.post('/login',(req,res) => {
    let user = users.find((user) => user.email == req.body.email)
    if(user){
        if(req.body.password == user.password){
            res.send({"error":0,"user":user.name})
        }else{
            res.send({"error":1})
        }
    }else{
        res.send({"error":1})
    }
    res.send(user)
})