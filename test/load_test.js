import http from "k6/http"
import {check,group} from "k6"
import postTest from "./api/post_test.js";

export const options = {
    // vus means virtual users
	vus:10,
    duration:"5m"
}

export default function(){
    group('postApi',()=>{
        postTest()
    })
}
