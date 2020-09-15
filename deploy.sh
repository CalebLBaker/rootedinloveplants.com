#!/bin/bash

gcloud builds submit --tag gcr.io/rootedinloveplants/rootedinloveplants
gcloud run deploy --image gcr.io/rootedinloveplants/rootedinloveplants --platform managed --port 8080

