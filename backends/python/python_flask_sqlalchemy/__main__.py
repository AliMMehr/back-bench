import logging
logging.basicConfig(level=logging.INFO)

def create_app_with_fallback():
    try:
        from .app import create_app
        return create_app()
    except:
        
        from flask import Flask
        logging.exception("Exception in create_app()")
        return Flask("fallback-flask-app")

app = create_app_with_fallback()

if __name__ == "__main__" :
    pass
