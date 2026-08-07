# Progetti Futuri e Roadmap

Questo file tiene traccia delle evoluzioni pianificate per l'immagine OS, con l'obiettivo di aumentare la sovranità tecnologica, la sicurezza e l'automazione.

## 1. Sovranità dei Pacchetti (Source Sovereignty)
L'obiettivo è ridurre drasticamente la dipendenza da repository COPR personali o di terze parti, preferendo l'acquisizione diretta dalle fonti ufficiali.

- [x] **Utility Rust nello Stage Builder**: Spostata l'acquisizione di `sudo-rs` e `uutils-coreutils` dai COPR/RPM direttamente ai repository GitHub ufficiali.
    - Utilizzo dello stage `builder` per scaricare i binari.
    - Implementata la verifica dei checksum (SHA256) per ogni binario scaricato.
- [ ] **Migrazione Utility DMS e Niri**: Spostare l'acquisizione di `niri` e `dms` dai COPR direttamente ai repository GitHub ufficiali (richiede valutazione compilazione vs binari).
- [ ] **Migrazione Kernel CachyOS**: Passare dai COPR personali al repository ufficiale gestito dal team di CachyOS (se disponibile per Fedora) o automatizzare il monitoraggio delle versioni ufficiali.

## 2. Automazione e Aggiornamenti
- [x] **Integrazione Dependabot per Stage Builder**: Configurato Dependabot (ecosistema `dockerfile`) per monitorare e aggiornare automaticamente le versioni delle `ARG` definite nello stage builder (`Containerfile`).
- [x] **Layer Caching Intelligente e Modularizzazione**: Implementata la suddivisione del `Containerfile` in 5 script modulari in `build_files/`, abilitando `--cache-from` per PR/push manuali e `--no-cache` per il cron giornaliero del mattino.

## 3. Ottimizzazioni e Sicurezza
- [x] **Riproducibilità delle Build e Update OCI Ottimizzati**: Condizionato `dracut` initramfs, configurato `IMAGE_ID` via Git `SHA_HEAD_SHORT` in `/usr/lib/os-release` ed esportato `SOURCE_DATE_EPOCH` per azzerare il download di aggiornamenti ridondanti su `bootc upgrade`.
- [ ] **Minimal Image**: Analizzare ulteriormente i pacchetti installati per rimuovere dipendenze legacy ereditate dall'immagine base Fedora, puntando a un'immagine ancora più snella e performante.

## 5. Miglioramento Esperienza di Login
L'obiettivo è rendere il processo di login più fluido e integrato con l'estetica del desktop.

- [x] **Transizione da `tuigreet` a `dms-greeter`**: Valutata la rimozione di `tuigreet` in favore del greeter grafico di Dank Material Shell.
    - Configurato `greetd` per avviare `dms-greeter`.
    - Verificata la corretta integrazione con `niri-session`.
