ARG IKA_TOOLS_IMAGE_TAG

FROM ika-io/ika-tools:$IKA_TOOLS_IMAGE_TAG AS setup

RUN apt update
RUN apt install python3 python3-pip -y

# copy configuration files to root
COPY ./new-genesis.sh /new-genesis.sh
COPY ./genesis /genesis

WORKDIR /

ARG IKA_NODE_A_TAG
ARG IKA_NODE_B_TAG
ENV IKA_NODE_A_TAG=$IKA_NODE_A_TAG
ENV IKA_NODE_B_TAG=$IKA_NODE_B_TAG

RUN ./new-genesis.sh
RUN echo "IKA_NODE_A_TAG=$IKA_NODE_A_TAG" >> /.env
RUN echo "IKA_NODE_B_TAG=$IKA_NODE_B_TAG" >> /.env

FROM scratch

COPY ./docker-compose-antithesis.yaml /docker-compose.yaml
COPY /genesis/overlays/* /genesis/overlays/
COPY /genesis/static/* /genesis/static/
COPY --from=setup /genesis/files/* /genesis/files/
COPY --from=setup /.env /.env

