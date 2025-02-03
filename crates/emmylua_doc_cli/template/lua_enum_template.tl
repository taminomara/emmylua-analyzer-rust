# enum {{ type_name }}
---
{% if namespace %}
- namespace: {{ namespace }}
{% endif %}

{% if description %}
{{ description }}
{% endif %}

```C++
enum {{ type_name }} {
{% for field in fields -%}
    {{ field.name }} = {{ field.value }}, {{ field.description }}
{% endfor %}
}
```
