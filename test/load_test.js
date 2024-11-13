import http from "k6/http"
import {check,group} from "k6"
import postTest from "./api/post_test.js";

export const options = {
    // vus means virtual users
    vus:500,
    duration:"5m"
}

export default function(){
    const res = http.get("http://api:8080/api/healthcheck");
    check(res, {
        "status is 200": (r) => r.status === 200,
    });
    group('postApi',()=>{
        postTest()
    })
}
