FROM python:3.10 as base

WORKDIR /backends/python

RUN addgroup group1
RUN adduser user1 --ingroup group1

RUN set -xe; \
    apt update -y; \
    apt upgrade -y; \
    apt install -y libpq-dev; \
    apt clean;

COPY docker/python_flask_sqlalchemy.requirements.txt ./docker/

RUN set -xe; \
    pip install -r ./docker/python_flask_sqlalchemy.requirements.txt;

USER user1:group1

ENV FLASK_APP_LOC 'python_flask_sqlalchemy.__main__:app'
ENV PYTHONUNBUFFERED '1'

FROM base as dev

WORKDIR /backends/python

RUN pip install debugpy;

CMD set -xe; \
    RELOAD_EXTRA_FILES_STR=$( find . -type f -name "*.py" | awk '{printf " --reload-extra-file %s", $0}'); \
    gunicorn --bind 0.0.0.0:3001 --log-level 'debug' --reload $RELOAD_EXTRA_FILES_STR -w 1 $FLASK_APP_LOC

FROM base as prod

WORKDIR /backends/python

COPY backends/python/sqlalchemy_db_models/ ./sqlalchemy_db_models/
COPY backends/python/python_flask_sqlalchemy/ ./flask_backend/
COPY backends/python/shared_python/ ./shared_python/

CMD gunicorn --bind 0.0.0.0:3001 $FLASK_APP_LOC ;
