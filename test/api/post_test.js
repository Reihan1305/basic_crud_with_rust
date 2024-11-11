import http from "k6/http"
import { check } from "k6"
import { uuidv4 } from "https://jslib.k6.io/k6-utils/1.2.0/index.js";

export default function postTest(){
    //getall api
    const resGetAll = http.get("http://api:8080/api/post");
    check(resGetAll,{
        "status is 200 get all":(r)=> r.status === 200,
    })

    
    const params = {
        headers:{
            "Content-Type":"application/json"
        }
    }

    //create api 
    const createPayload = JSON.stringify({title:`${uuidv4()} this is title`, content:`this is content ${uuidv4()}`})

    const resCreate =  http.post("http://api:8080/api/post",createPayload,params)

    check(resCreate,{
        "status is 201 created":(r)=> r.status === 201
    })

    const postId = resCreate.json().data.post.id

    //get detail

    const getOne = http.get(`http://api:8080/api/post/${postId}`)

    check(getOne,{
        "status is 200 get one": (r)=>r.status === 200
    })
    
    //update api
    const updatePayload = JSON.stringify({title:`${uuidv4()} updated post title`,content:"updated post content"})

    const resUpdated = http.patch(`http://api:8080/api/post/${postId}`,updatePayload,params)

    check(resUpdated,{
        "status is 200 updated":(r)=>r.status === 200
    })

    //delete api
    const resDeleted = http.del(`http://api:8080/api/post/${postId}`)

    check(resDeleted,{
        "status is 200 deleted":(r)=>r.status === 204
    })
}