import { Logging, Logger } from "jsfastlogging";

const pre = document.getElementById("jsfastlogging");

const logButton = document.getElementById("log");

let logging = Logging.init();

logButton.addEventListener("click", event => {
    logging.debug("Debug Message");
});
