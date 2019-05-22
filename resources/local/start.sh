API=true
STORAGE=true

SCRIPT_DIR=`cd $(dirname $0); pwd`
PROJECT_DIR=`cd "$SCRIPT_DIR/../../"; pwd`

function deploy() {
  echo "Starting deployment..."
  cd ${PROJECT_DIR}

  if [[ ${STORAGE} == "true" ]]; then
    echo "Deploying storage containers..."
    docker-compose -f resources/docker/storage.yaml up -d --build

    echo "Waiting for databases to start..."
    sleep 15

    echo "Seeding databases with data..."
    for sql_file in `ls test/resources/mysql/*.sql`; do mysql -h 127.0.0.1 -urassasy-user -ppassword < $sql_file ; done
    for cypher_file in `ls test/resources/cypher/*.cypher`; do cat $cypher_file | cypher-shell -u neo4j -p password --format plain ; done
  fi

  if [[ ${API} == "true" ]]; then
    echo "Deploying api containers..."
    docker-compose -f resources/docker/api.yaml up -d --build
  fi

  echo "Deployment complete!"
}

while [[ $# -gt 0 ]];
do
    case $1 in
        --no-api)
            shift
            API=false
            shift 2
        ;;
        --no-storage)
            shift
            STORAGE=false
            shift 2
        ;;
        -h | --help)
            help
            exit 0;
        ;;
        *)
            help
            exit 0;
        ;;
    esac
done

deploy