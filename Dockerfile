FROM danielgtaylor/apisprout@sha256:6c07143937e57095d8478efc8ab7eab52b44e67c7673285f8c0a2bf4a7b137ad

COPY openapi.yaml .

CMD ["openapi.yaml", "--validate-request"]
