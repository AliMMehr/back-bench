import logging
from flask import Flask

def create_app():
    app = Flask("python_flask_sqlalchemy")

    from . import blueprints
    app.register_blueprint(blueprints.bp)
    
    logging.info(f"url_map: {app.url_map}")

    return app
