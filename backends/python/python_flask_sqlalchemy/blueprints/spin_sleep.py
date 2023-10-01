from datetime import datetime, timedelta
from flask import Blueprint
import time

bp = Blueprint("spin_sleep", __name__, )

@bp.route("/spin_sleep")
def spin_sleep():
    end_wait = datetime.now() + timedelta(seconds=10)
    
    while True:
        time.sleep(0.1)
        if datetime.now() > end_wait:
            break
    
    return "Spin sleep!"
