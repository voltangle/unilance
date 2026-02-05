package com.arvenora.lancemate

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform