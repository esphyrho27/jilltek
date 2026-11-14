import logging, os, time
logging.basicConfig(level=os.getenv("LOG_LEVEL", "INFO"), format="%(asctime)s %(levelname)s %(message)s")
log = logging.getLogger("jilltek-agent")
def main():
    interval = int(os.getenv("AGENT_INTERVAL_SECONDS", "30"))
    log.info("Jilltek MikroTik agent starter running; no router connections or credentials configured.")
    while True:
        log.debug("Agent heartbeat")
        time.sleep(interval)
if __name__ == "__main__":
    main()
