// Generating an asymmetric key pair. The public one will be shared with other users, and the private one will be kept secret.
export async function generateKeyPair() {
    return await crypto.subtle.generateKey(
        {
            name: 'ECDH',
            namedCurve: 'P-384',
        },
        true,
        ['deriveKey', 'deriveBits'],
    )
}

// creating a symmetric key from the private key and the public key
export async function deriveSymmetricKey(privateKey, publicKey) {
    return await crypto.subtle.deriveKey(
        {
            name: 'ECDH',
            namedCurve: 'P-384',
            public: publicKey,
        },
        privateKey,
        { name: 'AES-GCM', length: 256 },
        true,
        ['encrypt', 'decrypt'],
    )
}

export async function encryptMessageBase64Representation(key, message) {
    const enc = new TextEncoder()
    const iv = crypto.getRandomValues(new Uint8Array(12)) // 96-bit IV for AES-GCM

    // Encrypt the message
    const ciphertext = await crypto.subtle.encrypt({ name: 'AES-GCM', iv: iv }, key, enc.encode(message))

    const base64Ciphertext = uint8ArrayToBase64(new Uint8Array(ciphertext))
    const base64Iv = uint8ArrayToBase64(iv)

    return { cipher: base64Ciphertext, iv: base64Iv }
}

export async function decryptMessageFromBase64Representation(key, base64Ciphertext, base64Iv, t) {
    // Convert base64 encoded data back to Uint8Array
    const ciphertext = base64ToUint8Array(base64Ciphertext)
    const iv = base64ToUint8Array(base64Iv)

    // Decrypt the message
    try {
        const decrypted = await crypto.subtle.decrypt({ name: 'AES-GCM', iv: iv }, key, ciphertext)
        const dec = new TextDecoder()

        return dec.decode(decrypted)
    } catch (error) {
        console.error('Failed to decrypt message:', error)
        return t('chat.security.error.decrypt')
    }
}

function uint8ArrayToBase64(bytes) {
    const binaryString = String.fromCharCode(...bytes)
    return btoa(binaryString)
}

function base64ToUint8Array(base64) {
    const binaryString = atob(base64)
    const len = binaryString.length
    const bytes = new Uint8Array(len)
    for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i)
    }
    return bytes
}

export async function exportKeyToJwk(key) {
    const export_key = await crypto.subtle.exportKey('jwk', key)
    // Setting key_ops explicitly to empty as of chrome 2024, otherwise it will throw an error
    if (!export_key.key_ops) {
        export_key.key_ops = []
    }

    return export_key
}

export async function exportSymmetricKeyToRaw(key) {
    const keyBuffer = await crypto.subtle.exportKey('raw', key)
    return uint8ArrayToBase64(new Uint8Array(keyBuffer))
}

export async function importKeyFromJwk(jwk) {
    // Setting key_ops explicitly to empty as of chrome 2024, otherwise it will throw an error
    jwk.key_ops = []

    // Check if all required fields are present
    if (!jwk.kty || !jwk.crv || !jwk.x || !jwk.y) {
        throw new Error('Invalid JWK structure: Missing required fields for ECDH key')
    }

    return await crypto.subtle.importKey('jwk', jwk, { name: 'ECDH', namedCurve: jwk.crv }, true, jwk.key_ops)
}

export async function importSymmetricKeyFromRaw(rawKey) {
    const keyBuffer = base64ToUint8Array(rawKey)

    return await crypto.subtle.importKey('raw', keyBuffer, { name: 'AES-GCM' }, true, ['encrypt', 'decrypt'])
}
