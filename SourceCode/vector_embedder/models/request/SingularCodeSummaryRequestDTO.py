from pydantic import BaseModel


class SingularCodeSummaryRequestDTO(BaseModel):
    text: str
