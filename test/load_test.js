import http from "k6/http"
import {check,group,sleep} from "k6"
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import postTest from "/home/reihan1305/my-project/rust/actix_basic_api/basic_crud/test/api/post_test.js";

export const options = {
    // vus means virtual users
    vus:100,
    duration:"1m"
}

export default function(){
    const res = http.get("http://127.0.0.1:8000/api/healthcheck");
    check(res, {
        "status is 200": (r) => r.status === 200,
    });
    group('postApi',()=>{
        postTest()
    })
    sleep(1);
}

export function handleSummary(data) {
    return {
      "report.html": htmlReport(data),
    };
}