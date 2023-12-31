from typing import List
import yaml


def yaml_to_markdown_table(file_path):
    # Read the YAML file
    with open(file_path, "r") as file:
        design_value_data = yaml.safe_load(file)

    # check the Yaml format is a list of dictionaires, other format are possible
    if (
        not design_value_data
        or not isinstance(design_value_data, list)
        or not all(isinstance(item, dict) for item in design_value_data)
    ):
        return "Invalid YAML format for table generation."

    markdown = ""
    for section in design_value_data:
        for section_name, section_data in section.items():
            markdown += createTable(section_name, section_data)

    return markdown


def createTable(table_name, table_values) -> str:
    # add section headers
    markdown = f"# {table_name} \n"
    # header set is union of attribute field names
    table_headers = get_header_set_as_list(table_values)

    # alphabetical sort
    table_headers.sort()
    markdown += "| " + " | ".join(table_headers) + " |\n"
    markdown += "| " + " | ".join(["---"] * len(table_headers)) + " |\n"
    for alloy_name, attribute_feilds in table_values.items():
        markdown += gen_table_line(alloy_name, attribute_feilds, table_headers) + "\n"
    return markdown


def write_to_file(s: str):
    with open("book.md", "w") as f:
        f.write(s)


def get_header_set_as_list(v) -> List[str]:
    header_set = set()
    for value in v.values():
        for k in value.keys():
            header_set.add(k)
    return list(header_set)


def gen_table_line(alloy_name: str, attribute_feilds: dict, headers) -> str:
    line = f"| "
    for header in headers:
        if header in attribute_feilds:
            line += f" {str(attribute_feilds[header]).strip()} |"
        else:
            line += f" |"
    return line


if __name__ == "__main__":
    md = yaml_to_markdown_table("./designvalues.yaml")
    write_to_file(md)
