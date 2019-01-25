FROM docker.elastic.co/elasticsearch/elasticsearch:6.5.4

RUN elasticsearch-plugin remove x-pack

EXPOSE 9200
