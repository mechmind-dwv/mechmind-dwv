FROM python:3.11-slim-bullseye
WORKDIR /app
COPY . .
RUN pip install -r requirements.txt
EXPOSE 5000
CMD ["python", "mechcore/main.py"]
