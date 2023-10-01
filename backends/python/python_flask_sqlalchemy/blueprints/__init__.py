from flask import Blueprint

from . import (
    hello, 
    thread_sleep,
    spin_sleep,
)

bp = Blueprint("blueprints", __name__, )
bp.register_blueprint(hello.bp)
bp.register_blueprint(thread_sleep.bp)
bp.register_blueprint(spin_sleep.bp)
