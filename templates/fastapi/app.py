# flake8: noqa E501
import os
import logging
from fastapi import FastAPI, Body

fastapi_env = os.environ.get("FASTAPI_ENV", "development")
logger = logging.getLogger('uvicorn')
app = FastAPI()


@app.get("/")
async def root():
    return {"message": "hello"}


@app.get("/ping")
async def ping():
    return {"pong"}
