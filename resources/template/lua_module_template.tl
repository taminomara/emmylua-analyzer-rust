# {{ module_name }}

{% if description %}
{{ description }}
{% endif %}
---
{% if methods %}
## methods
---
{% for method in methods %}
### {{ method.name }}
---
```lua
{{ method.display }}
```

{% if method.description %}
{{ method.description }}
{% endif %}
{% endfor %}
{% endif %}

{% if fields %}
## fields
---
{% for field in fields %}
### {{ field.name }}
---
```lua
{{ field.display }}
```

{% if field.description %}
{{ field.description }}
{% endif %}
{% endfor %}
{% endif %}