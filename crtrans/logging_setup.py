import logging
from logging.handlers import RotatingFileHandler
from pathlib import Path


def setup_logging(log_dir: Path, level: str = "INFO") -> None:
    log_dir.mkdir(parents=True, exist_ok=True)
    logfile = log_dir / "crtrans.log"

    logger = logging.getLogger()
    if logger.handlers:
        return

    logger.setLevel(getattr(logging, level.upper(), logging.INFO))

    formatter = logging.Formatter(
        "%(asctime)s [%(levelname)s] %(name)s - %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )

    ch = logging.StreamHandler()
    ch.setLevel(getattr(logging, level.upper(), logging.INFO))
    ch.setFormatter(formatter)
    logger.addHandler(ch)

    fh = RotatingFileHandler(logfile, maxBytes=2 * 1024 * 1024, backupCount=3)
    fh.setLevel(logging.DEBUG)  # capture verbose details in file only
    fh.setFormatter(formatter)
    logger.addHandler(fh)
