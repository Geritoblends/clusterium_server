# Clusterium server manifest
1. Network protocol -> client-server communication + protocol endpoints (/trade, /confirm)
    * Client-server
    - /connect:
        The endpoint to request a connection to a socket room
    - /request_creation: Ledger.Operaciones.Create() request

2. Ledger
    - Schema
        { 
        "account_id", // ID de la cuenta, hash indexado junto con item_uuid
        "action", // CREATE | DESTROY | SPLIT // Probablemente irrelevante
        "item_category_id", // Id del item (por ejemplo "world_lock"), hash indexado junto con account_id e item_uuid
        "qty", // Cantidad recibida (negativa si está transfiriendo o es un DESTROY)
        "balance", //Calculado como el último balance de dicho item del account_id, + qty
        "item_uuid", // rastreador del item o stack
        "sequence_number", // Un número entero secuencial que va aumentando en 1 cada vez que hay una nueva entry en el ledger con el mismo item_uuid
        "uuid", // Llave primaria, hash de item_uuid, account_id y sequence_number. Al insertar una nueva entry, se especifica ON CONFLICT DO NOTHING (loggearlo en vez de no hacer nada)
        "active", // Se desactiva cuando balance = 0, para descartar dichas entries en las búsquedas.
        }
    - Operaciones:
        * Create
           Creará un nuevo ledger entry con un nuevo item_uuid, y sequence_number = 1
        * Destroy
            Quitará una qty de un stack ya existente, y se incrementará el sequence_number
        * Split
            Dicho item se dividirá en dos stacks con el mismo item_uuid y sequence_number a través de dos diferentes cuentas. Habrá dos inserts, uno para la account_id que anteriormente tenia el stack, con una qty negativa, y el segundo para la cuenta que esta recibiendo el stack, con una qty positiva, el mismo item_uuid, y el sequence_number correspondiente.
            
