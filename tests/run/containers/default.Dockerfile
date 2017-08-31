FROM docker.elastic.co/elasticsearch/elasticsearch:5.5.0

RUN elasticsearch-plugin remove x-pack

EXPOSE 9200
