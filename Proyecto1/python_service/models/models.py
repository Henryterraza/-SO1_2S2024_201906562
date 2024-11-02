from pydantic import BaseModel # type: ignore
from typing import List

class LogProcess(BaseModel):
    pid: int
    container_id: str
    name: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float
    date: str

class LogMemor(BaseModel):
    total_ram: int
    ram_libre: int
    ram_usado: int
    date: str
