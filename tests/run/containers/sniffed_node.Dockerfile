FROM docker.elastic.co/elasticsearch/elasticsearch:6.5.4

ADD sniffed_node.elasticsearch.yml /usr/share/elasticsearch/config/elasticsearch.yml
USER root
RUN chown elasticsearch:elasticsearch config/elasticsearch.yml
USER elasticsearch

EXPOSE 9200
