## Wallet que contiene los puntos conseguidos para el challenge 1 y 2 es:

`erd1a9wdz7pdyttectxjntg4z83unkal7pj5ycfem5ynzh49j8st4mas52yhel`

## Waller con la mayoria de contratos desplegados para intentar resolver los challenges:

`erd139adwtgs4smel4rmqlphcxp8q8jr5qsxrfwlluhzvds3elk3hm2qq3vnnc`

# Bump

```
mxpy contract query erd1qqqqqqqqqqqqqpgq23j27f6w0r75hfyc5td753f9ahvfpp5x4wzq65czqw \
    --function="bumps" \
    --arguments erd1a9wdz7pdyttectxjntg4z83unkal7pj5ycfem5ynzh49j8st4mas52yhel \
    --proxy=https://devnet-gateway.multiversx.com
```

# Coinflip

```
mxpy contract query erd1qqqqqqqqqqqqqpgq0jtfsyk7rfgu50v6wh5wwtk2mjgseca54wzqyntf2v \
    --function="bumps" \
    --arguments erd1a9wdz7pdyttectxjntg4z83unkal7pj5ycfem5ynzh49j8st4mas52yhel \
    --proxy=https://devnet-gateway.multiversx.com
```

# Gaspass

No logré completar el challenge 3 utilizando el contrato que se nos proporcionó. Sin embargo, sí lo conseguí con el contrato del repositorio, donde repliqué el mismo código pero agregando event logs. Sospecho que estos logs consumen un poco más de gas, lo que podría estar afectando el resultado.

La estrategia que seguí fue:

Implementé un event log justo después de obtener el gas_left inicial para poder ver el valor exacto.

A través de múltiples pruebas, descubrí que:

El gas_left debe ser exactamente 3003960 (para el valor de the_key en el caso de mi address)

Si envío X cantidad de gas y obtengo un gas_left menor a 3003960, necesito enviar más gas. Si obtengo un gas_left mayor a 3003960, necesito enviar menos gas.

Finalmente encontré que enviando 4278630 gas, el gas_left coincidía exactamente con the_key, haciendo que passed sea true

A pesar de que esta estrategia funcionó perfectamente en mi SC con logs, el mismo valor de gas no funcionó en el contrato original sin logs, lo que sugiere que hay diferencias en el manejo del gas entre ambas implementaciones.

```
mxpy contract query erd1qqqqqqqqqqqqqpgqepljgh3a7putzvrcz34ms3na042dtvlchm2qrdxxnk \
    --function="bumps" \
    --arguments erd1a9wdz7pdyttectxjntg4z83unkal7pj5ycfem5ynzh49j8st4mas52yhel \
    --proxy=https://devnet-gateway.multiversx.com
```
