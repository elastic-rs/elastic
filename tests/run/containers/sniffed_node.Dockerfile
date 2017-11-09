FROM docker.elastic.co/elasticsearch/elasticsearch:5.5.0

ADD sniffed_node.elasticsearch.yml /usr/share/elasticsearch/config/elasticsearch.yml
USER root
RUN chown elasticsearch:elasticsearch config/elasticsearch.yml
USER elasticsearch

RUN elasticsearch-plugin remove x-pack

EXPOSE 9200
