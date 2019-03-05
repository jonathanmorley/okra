FROM danielgtaylor/apisprout

COPY openapi.yaml .

CMD ["openapi.yaml", "--validate-request"]
