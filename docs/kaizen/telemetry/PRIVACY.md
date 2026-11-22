# Telemetry privacy / Privacidad de la telemetria

This is a control list, not a legal opinion. A person must complete it before
any collection is enabled. Esta es una lista de control, no una opinion legal.
Una persona debe completarla antes de activar la recoleccion.

- [ ] Explicit, revocable opt-in documented for the end user.
      Opt-in explicito y revocable, documentado para el usuario final.
- [ ] No personal data: no names, emails, IPs, file paths, user content or
      tokens. Sin datos personales: nombres, correos, IPs, rutas, contenido ni
      tokens.
- [ ] Random rotating session identifiers. Identificadores aleatorios y
      rotativos.
- [ ] `attrs` keys are allowlisted; everything else is dropped in the SDK.
      Claves de `attrs` en lista permitida; todo lo demas se descarta en el SDK.
- [ ] Maximum retention defined: ___ days. Automatic deletion.
      Retencion maxima definida: ___ dias. Borrado automatico.
- [ ] Telemetry store is read-only for the loop.
      Acceso de solo lectura al almacen para el loop.
- [ ] Privacy or legal review completed by: ___ on date ___.
      Revision de privacidad realizada por: ___ en fecha ___.
