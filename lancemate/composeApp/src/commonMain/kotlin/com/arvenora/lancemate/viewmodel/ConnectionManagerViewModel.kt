package com.arvenora.lancemate.viewmodel

import androidx.compose.material3.SnackbarHostState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.lifecycle.ViewModel
import com.arvenora.lancemate.viewmodel.DeviceConnectionType.*
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.any
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.forEach
import kotlinx.coroutines.flow.update

enum class ConnectionMethod {
    BLE, TCP, USB
}

enum class ConnectionState {
    NotConnected, Connecting, Connected
}

data class Device(
    var name: String,
    var connection: DeviceConnectionType,
    var state: ConnectionState
)

sealed class DeviceConnectionType {
    // TODO: add params
    class Bluetooth() : DeviceConnectionType()
    class TCP() : DeviceConnectionType()
    class USB() : DeviceConnectionType()
}

class ConnectionManagerViewModel(val snackbarHostState: SnackbarHostState) : ViewModel() {
    var method by mutableStateOf(ConnectionMethod.BLE)
    var showConnectionSheet by mutableStateOf(false)
    var showLoading by mutableStateOf(false)
    var anyConnectedDevices by mutableStateOf(false)

    var devicesFlow = MutableStateFlow(listOf(
        Device("TOLLSHIFT", Bluetooth(), ConnectionState.NotConnected),
        Device("Begode ET Max", Bluetooth(), ConnectionState.NotConnected)
    ))
    var devices = devicesFlow.asStateFlow()

    suspend fun connectToDevice(device: Device) {
        // Okay, here's the deal. I understand that just looping over every device is maybe
        // NOT the best solution there could possibly be. I totally get that, and I agree
        // with whoever is currently cursing me for writing this. My argument is that
        // the lists are never going to be that huge that this will ACTUALLY have a performance
        // impact, considering that this is a suspend function and doesn't block UI. This
        // is just the easiest way to do it. Thank you for understanding.
        devicesFlow.update { device ->
            device.map {
                if (it == device) {
                    it.state = ConnectionState.Connecting
                }
                it
            }
        }
        showLoading = true
        delay(1500) // doing some work
        devicesFlow.update { device ->
            device.map {
                // TODO: I should either do a custom compare function or add some kind of ID
                if (it == device) {
                    it.state = ConnectionState.Connected
                }
                it
            }
        }
        anyConnectedDevices = true
        showLoading = false
        snackbarHostState.showSnackbar("Connected to ${device.name}", withDismissAction = true)
    }

    suspend fun showMessage(message: String) {
        snackbarHostState.showSnackbar(message, withDismissAction = true)
    }

    suspend fun disconnectFromDevice(device: Device) {
        showLoading = true
        delay(1000)
        devicesFlow.update { device ->
            device.map {
                if (it == device) {
                    it.state = ConnectionState.NotConnected
                }
                it
            }
        }
        anyConnectedDevices = devicesFlow.any { list -> list.any { it.state == ConnectionState.Connected } }
        showLoading = false
        snackbarHostState.showSnackbar("Disconnected from ${device.name}", withDismissAction = true)
        // devices.retainAll { it != device }
    }

    fun hideSheet() {
        showConnectionSheet = false
    }

    fun showSheet() {
        showConnectionSheet = true
    }
}