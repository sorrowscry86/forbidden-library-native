<script lang="ts">
	import { onMount } from 'svelte';
	import { safeInvoke, isTauriAvailable } from '$lib/utils/enhanced-tauri-detection';
	import { errorStore } from '$lib/stores/enhanced-error-store';

	let systemInfo: any = null;
	let isDesktop = false;
	let windowState: any = null;
	let alwaysOnTop = false;
	let isDarkMode = false;
	let clipboardContent = '';
	let updateInfo: any = null;
	let loading = false;

	onMount(async () => {
		isDesktop = isTauriAvailable();
		if (isDesktop) {
			await loadDesktopInfo();
		}
	});

	async function loadDesktopInfo() {
		try {
			loading = true;
			
			// Load system information
			systemInfo = await safeInvoke('get_system_info');
			
			// Load window state
			windowState = await safeInvoke('get_window_state');
			
			// Check dark mode
			isDarkMode = await safeInvoke('is_dark_mode');
			
			// Check for updates
			updateInfo = await safeInvoke('check_for_updates');
			
		} catch (error) {
			console.error('Failed to load desktop info:', error);
			errorStore.addError({
				message: 'Failed to load desktop information',
				details: error instanceof Error ? error.message : String(error),
				category: 'API' as any,
				severity: 'WARNING' as any
			});
		} finally {
			loading = false;
		}
	}

	async function showNotification() {
		try {
			await safeInvoke('show_notification', {
				title: 'Forbidden Library',
				body: 'This is a desktop notification!',
				icon: null
			});
		} catch (error) {
			console.error('Failed to show notification:', error);
		}
	}

	async function openFileDialog() {
		try {
			const result = await safeInvoke('show_open_dialog', {
				title: 'Select a file',
				default_path: null,
				filters: [['Text Files', ['txt', 'md']], ['All Files', ['*']]]
			});
			
			if (result) {
				console.log('Selected file:', result);
				alert(`Selected file: ${result}`);
			}
		} catch (error) {
			console.error('Failed to open file dialog:', error);
		}
	}

	async function saveFileDialog() {
		try {
			const result = await safeInvoke('show_save_dialog', {
				title: 'Save file as',
				default_path: 'conversation.txt',
				filters: [['Text Files', ['txt']], ['Markdown Files', ['md']]]
			});
			
			if (result) {
				console.log('Save to:', result);
				
				// Write a sample file
				await safeInvoke('write_file_to_disk', {
					path: result,
					content: 'This is a sample conversation export from Forbidden Library desktop app.\n\nThis demonstrates native file system access!'
				});
				
				alert(`File saved to: ${result}`);
			}
		} catch (error) {
			console.error('Failed to save file:', error);
		}
	}

	async function copyToClipboard() {
		try {
			const text = 'This text was copied using native desktop APIs!';
			await safeInvoke('copy_to_clipboard', { text });
			alert('Text copied to clipboard!');
		} catch (error) {
			console.error('Failed to copy to clipboard:', error);
		}
	}

	async function readFromClipboard() {
		try {
			clipboardContent = await safeInvoke('read_from_clipboard');
		} catch (error) {
			console.error('Failed to read from clipboard:', error);
		}
	}

	async function toggleAlwaysOnTop() {
		try {
			alwaysOnTop = !alwaysOnTop;
			await safeInvoke('set_window_always_on_top', { always_on_top: alwaysOnTop });
		} catch (error) {
			console.error('Failed to toggle always on top:', error);
		}
	}

	async function minimizeToTray() {
		try {
			await safeInvoke('minimize_to_tray');
		} catch (error) {
			console.error('Failed to minimize to tray:', error);
		}
	}

	async function openUrl() {
		try {
			await safeInvoke('open_external_url', { url: 'https://github.com/sorrowscry86/forbidden-library' });
		} catch (error) {
			console.error('Failed to open URL:', error);
		}
	}

	async function createDesktopShortcut() {
		try {
			await safeInvoke('create_desktop_shortcut');
			alert('Desktop shortcut created!');
		} catch (error) {
			console.error('Failed to create desktop shortcut:', error);
		}
	}

	async function getAppDataDir() {
		try {
			const dir = await safeInvoke('get_app_data_dir');
			alert(`App data directory: ${dir}`);
		} catch (error) {
			console.error('Failed to get app data dir:', error);
		}
	}
</script>

<div class="desktop-features">
	<div class="header">
		<h2>🖥️ Desktop Features</h2>
		<p class="status">
			{#if isDesktop}
				<span class="badge success">✓ Desktop Mode</span>
			{:else}
				<span class="badge warning">⚠ Web Mode - Limited Features</span>
			{/if}
		</p>
	</div>

	{#if !isDesktop}
		<div class="web-notice">
			<h3>🌐 Running in Web Mode</h3>
			<p>You're currently using the web version. Download and install the desktop application to access these native features:</p>
			<ul>
				<li>Native file system access</li>
				<li>System notifications</li>
				<li>Clipboard integration</li>
				<li>Window management</li>
				<li>System tray functionality</li>
				<li>Global shortcuts</li>
				<li>Offline storage with encryption</li>
			</ul>
		</div>
	{:else}
		{#if loading}
			<div class="loading">Loading desktop features...</div>
		{:else}
			<!-- System Information -->
			{#if systemInfo}
				<div class="feature-group">
					<h3>📊 System Information</h3>
					<div class="info-grid">
						<div class="info-item">
							<strong>Platform:</strong> {systemInfo.platform}
						</div>
						<div class="info-item">
							<strong>OS:</strong> {systemInfo.os}
						</div>
						<div class="info-item">
							<strong>Architecture:</strong> {systemInfo.arch}
						</div>
						<div class="info-item">
							<strong>Version:</strong> {systemInfo.version}
						</div>
					</div>
				</div>
			{/if}

			<!-- File System Operations -->
			<div class="feature-group">
				<h3>📁 Native File System</h3>
				<div class="button-grid">
					<button on:click={openFileDialog} class="feature-button">
						📂 Open File Dialog
					</button>
					<button on:click={saveFileDialog} class="feature-button">
						💾 Save File Dialog
					</button>
					<button on:click={getAppDataDir} class="feature-button">
						📁 App Data Directory
					</button>
				</div>
			</div>

			<!-- System Integration -->
			<div class="feature-group">
				<h3>🔔 System Integration</h3>
				<div class="button-grid">
					<button on:click={showNotification} class="feature-button">
						🔔 Show Notification
					</button>
					<button on:click={copyToClipboard} class="feature-button">
						📋 Copy to Clipboard
					</button>
					<button on:click={readFromClipboard} class="feature-button">
						📄 Read Clipboard
					</button>
					<button on:click={openUrl} class="feature-button">
						🌐 Open External URL
					</button>
				</div>
				
				{#if clipboardContent}
					<div class="clipboard-content">
						<strong>Clipboard Content:</strong>
						<pre>{clipboardContent}</pre>
					</div>
				{/if}
			</div>

			<!-- Window Management -->
			<div class="feature-group">
				<h3>🪟 Window Management</h3>
				<div class="button-grid">
					<button 
						on:click={toggleAlwaysOnTop} 
						class="feature-button {alwaysOnTop ? 'active' : ''}"
					>
						📌 Always On Top {alwaysOnTop ? '(ON)' : '(OFF)'}
					</button>
					<button on:click={minimizeToTray} class="feature-button">
						⬇️ Minimize to Tray
					</button>
				</div>
				
				{#if windowState}
					<div class="window-info">
						<strong>Window State:</strong>
						<div class="info-grid">
							<div class="info-item">Size: {windowState.width}x{windowState.height}</div>
							<div class="info-item">Position: ({windowState.x}, {windowState.y})</div>
							<div class="info-item">Maximized: {windowState.maximized ? 'Yes' : 'No'}</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Desktop Integration -->
			<div class="feature-group">
				<h3>🖥️ Desktop Integration</h3>
				<div class="button-grid">
					<button on:click={createDesktopShortcut} class="feature-button">
						🔗 Create Desktop Shortcut
					</button>
					<div class="theme-info">
						<span class="badge {isDarkMode ? 'dark' : 'light'}">
							{isDarkMode ? '🌙 Dark Mode' : '☀️ Light Mode'}
						</span>
					</div>
				</div>
			</div>

			<!-- Update Information -->
			{#if updateInfo}
				<div class="feature-group">
					<h3>🔄 Updates</h3>
					<div class="update-info">
						<div class="info-item">
							<strong>Current Version:</strong> {updateInfo.current_version}
						</div>
						<div class="info-item">
							<strong>Latest Version:</strong> {updateInfo.latest_version}
						</div>
						<div class="info-item">
							<span class="badge {updateInfo.available ? 'warning' : 'success'}">
								{updateInfo.available ? '⬆️ Update Available' : '✅ Up to Date'}
							</span>
						</div>
					</div>
				</div>
			{/if}
		{/if}
	{/if}
</div>

<style>
	.desktop-features {
		padding: 2rem;
		max-width: 800px;
		margin: 0 auto;
	}

	.header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.header h2 {
		margin: 0 0 0.5rem 0;
		color: var(--primary-color, #4f46e5);
	}

	.status {
		margin: 0;
	}

	.badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 1rem;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.badge.success {
		background-color: #10b981;
		color: white;
	}

	.badge.warning {
		background-color: #f59e0b;
		color: white;
	}

	.badge.dark {
		background-color: #374151;
		color: white;
	}

	.badge.light {
		background-color: #f3f4f6;
		color: #374151;
	}

	.web-notice {
		background-color: #fef3c7;
		border: 1px solid #f59e0b;
		border-radius: 0.5rem;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.web-notice h3 {
		margin: 0 0 1rem 0;
		color: #92400e;
	}

	.web-notice p {
		margin: 0 0 1rem 0;
		color: #92400e;
	}

	.web-notice ul {
		margin: 0;
		padding-left: 1.5rem;
		color: #92400e;
	}

	.loading {
		text-align: center;
		padding: 2rem;
		color: #6b7280;
		font-style: italic;
	}

	.feature-group {
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.feature-group h3 {
		margin: 0 0 1rem 0;
		color: #374151;
	}

	.button-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.feature-button {
		background-color: #4f46e5;
		color: white;
		border: none;
		border-radius: 0.375rem;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.feature-button:hover {
		background-color: #4338ca;
	}

	.feature-button.active {
		background-color: #10b981;
	}

	.feature-button.active:hover {
		background-color: #059669;
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 0.75rem;
	}

	.info-item {
		background-color: white;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		padding: 0.75rem;
		font-size: 0.875rem;
	}

	.clipboard-content,
	.window-info,
	.update-info {
		margin-top: 1rem;
		padding: 1rem;
		background-color: white;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
	}

	.clipboard-content pre {
		margin: 0.5rem 0 0 0;
		padding: 0.5rem;
		background-color: #f3f4f6;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		overflow-x: auto;
	}

	.theme-info {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	@media (max-width: 768px) {
		.desktop-features {
			padding: 1rem;
		}

		.button-grid {
			grid-template-columns: 1fr;
		}

		.info-grid {
			grid-template-columns: 1fr;
		}
	}
</style>