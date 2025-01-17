# {{ type_name }}

- supers: {{ super_types }}
- namespace: {{ namespace }}

{% if description %}
{{ description }}
{% endif %}

## methods
{% for method in methods %}
<span style="font-family: monospace;">`{{ method.display }}`</span>

{% if method.description %}
<span style="margin-left: 20px; font-size: 0.9em;">{{ method.description }}</span>
{% endif %}
{% endfor %}

## fields
{% for field in fields %}
<span style="font-family: monospace;">`{{ field.display }}`</span>

{% if field.description %}
<span style="margin-left: 20px; font-size: 0.9em;">{{ field.description }}</span>
{% endif %}
{% endfor %}
