FROM debian:alpine

ENV LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/app/
ENV PATH=/app/bin:${PATH}
ADD ./dist/ /app

RUN apt-get -y update
RUN apt-get -y upgrade

CMD ["application"]