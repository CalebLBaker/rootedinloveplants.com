#!/bin/bash

gcloud builds submit --tag gcr.io/rootedinloveplants/rootedinloveplants --timeout=20m && \
gcloud run deploy --image gcr.io/rootedinloveplants/rootedinloveplants --platform managed --port 8080

