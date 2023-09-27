from flask import Blueprint

from . import hello

bp = Blueprint("blueprints", __name__, )
bp.register_blueprint(hello.bp)
