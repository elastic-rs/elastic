# Client integration tests

This directory contains integration tests.

Tests are executed with a particular client against an Elasticsearch node hosted by `docker`.
The idea is to be able to run the same suite of tests against clusters with different configurations and make sure everything works as expected.

These tests only use the async client.
