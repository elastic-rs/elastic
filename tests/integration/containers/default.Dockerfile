FROM docker.elastic.co/elasticsearch/elasticsearch:7.2.0

ADD default.elasticsearch.yml /usr/share/elasticsearch/config/elasticsearch.yml
USER root
RUN chown elasticsearch:elasticsearch config/elasticsearch.yml
USER elasticsearch

EXPOSE 9200
