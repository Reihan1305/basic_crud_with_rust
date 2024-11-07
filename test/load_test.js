import http from "k6/http"
import {check,group} from "k6"
import postTest from "/home/reihan1305/my-project/rust/actix_basic_api/basic_crud/test/api/post_test.js";

export const options = {
    // vus means virtual users
    vus:5,
    duration:"1m",
    ext: {
        prometheus: {
          remote_write: "http://172.21.73.57:9090/api/v1/write",
        },
      },
}

export default function(){
    const res = http.get("http://127.0.0.1:8000/api/healthcheck");
    check(res, {
        "status is 200": (r) => r.status === 200,
    });
    group('postApi',()=>{
        postTest()
    })
}
