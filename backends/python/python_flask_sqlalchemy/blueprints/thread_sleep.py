from flask import Blueprint
import time

bp = Blueprint("thread_sleep", __name__, )

@bp.route("/thread_sleep")
def thread_sleep():
    time.sleep(10)
    return "Thread sleep!"
