<script lang="ts">
    import { onMount } from 'svelte';
    import { invokeWithTimeout, ms } from '$lib/services/api';
    import type { Project, ProjectMilestone } from '$lib/types/models';
    
    let projects: Project[] = [];
    let loading = true;
    let currentProject: Project | null = null;
    let newMilestoneTitle = '';
    
    onMount(async () => {
        await loadProjects();
    });
    
    async function loadProjects() {
        try {
            loading = true;
            projects = await invokeWithTimeout<Project[]>('get_projects', {}, ms(8));
            
            if (projects.length > 0) {
                currentProject = projects[0];
            }
        } catch (error) {
            console.error('Failed to load projects:', error);
            // Demo mode fallback
            projects = [
                {
                    id: 1,
                    name: 'Forbidden Library',
                    description: 'Privacy-first, high-performance desktop application for interacting with powerful language models',
                    repository_url: 'https://github.com/voidcat-rdc/forbidden-library',
                    status: 'Active',
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString(),
                    metadata: {
                        technology_stack: ['Rust', 'Tauri', 'SvelteKit', 'TypeScript', 'SQLite'],
                        team_members: ['Albedo', 'Codey Jr.', 'Pandora'],
                        milestones: [
                            {
                                id: '1',
                                title: 'Initial Repository Setup',
                                description: 'Create basic project structure and configuration',
                                due_date: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
                                completed: true,
                                completed_at: new Date().toISOString()
                            },
                            {
                                id: '2',
                                title: 'Core UI Implementation',
                                description: 'Implement the main chat interface and conversation management',
                                due_date: new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString(),
                                completed: false,
                                completed_at: null
                            },
                            {
                                id: '3',
                                title: 'AI Integration',
                                description: 'Connect to AI providers and implement conversation handling',
                                due_date: new Date(Date.now() + 21 * 24 * 60 * 60 * 1000).toISOString(),
                                completed: false,
                                completed_at: null
                            }
                        ],
                        repository_stats: {
                            total_commits: 42,
                            total_lines: 8750,
                            languages: {
                                'Rust': 4500,
                                'TypeScript': 2800,
                                'Svelte': 1200,
                                'CSS': 250
                            },
                            last_commit_date: new Date().toISOString(),
                            contributors: ['Albedo', 'Codey Jr.', 'Pandora']
                        }
                    }
                }
            ];
            currentProject = projects[0];
        } finally {
            loading = false;
        }
    }
    
    async function addMilestone() {
        if (!newMilestoneTitle.trim() || !currentProject?.id) return;
        
        try {
            const milestone: ProjectMilestone = {
                id: Date.now().toString(),
                title: newMilestoneTitle,
                description: null,
                due_date: new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString(),
                completed: false,
                completed_at: null
            };
            
            // In a real implementation, this would call the backend
            if (currentProject.metadata) {
                currentProject.metadata.milestones = [
                    ...currentProject.metadata.milestones,
                    milestone
                ];
            }
            
            newMilestoneTitle = '';
        } catch (error) {
            console.error('Failed to add milestone:', error);
        }
    }
    
    function toggleMilestoneStatus(milestoneId: string) {
        if (!currentProject?.metadata?.milestones) return;

        const milestoneIndex = currentProject.metadata.milestones.findIndex((m: ProjectMilestone) => m.id === milestoneId);
        if (milestoneIndex === -1) return;

        const milestone = currentProject.metadata.milestones[milestoneIndex];
        const updatedMilestone = {
            ...milestone,
            completed: !milestone.completed,
            completed_at: !milestone.completed ? new Date().toISOString() : null
        };

        currentProject.metadata.milestones[milestoneIndex] = updatedMilestone;
        currentProject = {...currentProject}; // Trigger reactivity
    }

    async function createNewProject() {
        const projectName = prompt('Enter project name:');
        if (!projectName?.trim()) return;

        try {
            // In a real implementation, this would call the backend
            const newProject: Project = {
                id: Date.now(),
                name: projectName,
                description: 'New project description',
                repository_url: null,
                status: 'Active',
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString(),
                metadata: {
                    technology_stack: [],
                    team_members: [],
                    milestones: [],
                    repository_stats: {
                        total_commits: 0,
                        total_lines: 0,
                        languages: {},
                        last_commit_date: new Date().toISOString(),
                        contributors: []
                    }
                }
            };

            projects = [...projects, newProject];
            currentProject = newProject;
        } catch (error) {
            console.error('Failed to create project:', error);
            alert('Failed to create project. Please try again.');
        }
    }
</script>

<div class="flex flex-col h-full">
    <!-- Project Header -->
    <div class="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div class="flex items-center justify-between">
            <div class="flex items-center space-x-4">
                <a
                    href="/"
                    class="p-2 text-gray-400 hover:text-white transition-colors"
                    title="Back to Conversations"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
                    </svg>
                </a>
                <div>
                    <h2 class="text-lg font-semibold text-white">Software Planning</h2>
                    <p class="text-sm text-gray-400">
                        {projects.length} projects available
                    </p>
                </div>
            </div>

            <div class="flex items-center space-x-2">
                <button
                    class="p-2 text-gray-400 hover:text-white transition-colors"
                    title="Create new project"
                    on:click={createNewProject}
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                    </svg>
                </button>

                <button
                    class="p-2 text-gray-400 hover:text-white transition-colors"
                    title="Refresh projects"
                    on:click={loadProjects}
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                    </svg>
                </button>
            </div>
        </div>
    </div>
    
    <!-- Project Content -->
    <div class="flex-1 overflow-y-auto">
        {#if loading}
            <div class="flex justify-center py-8">
                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-purple-600"></div>
            </div>
        {:else if !currentProject}
            <div class="flex flex-col items-center justify-center h-full text-gray-400 p-8">
                <div class="w-16 h-16 bg-gray-700 rounded-full flex items-center justify-center mb-4">
                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                    </svg>
                </div>
                <p class="text-lg font-medium mb-2">No projects available</p>
                <p class="text-sm text-gray-500">Create a new project to get started</p>
            </div>
        {:else}
            <div class="p-6">
                <!-- Project Overview -->
                <div class="bg-gray-700 rounded-lg p-6 mb-6">
                    <h3 class="text-xl font-semibold text-white mb-2">{currentProject.name}</h3>
                    <p class="text-gray-300 mb-4">{currentProject.description}</p>
                    
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                        <div class="bg-gray-800 p-4 rounded-lg">
                            <h4 class="text-sm font-medium text-gray-400 mb-1">Status</h4>
                            <p class="text-white font-semibold">{currentProject.status}</p>
                        </div>
                        
                        <div class="bg-gray-800 p-4 rounded-lg">
                            <h4 class="text-sm font-medium text-gray-400 mb-1">Created</h4>
                            <p class="text-white font-semibold">{new Date(currentProject.created_at).toLocaleDateString()}</p>
                        </div>
                        
                        <div class="bg-gray-800 p-4 rounded-lg">
                            <h4 class="text-sm font-medium text-gray-400 mb-1">Repository</h4>
                            <p class="text-white font-semibold truncate">
                                {currentProject.repository_url || 'Not specified'}
                            </p>
                        </div>
                    </div>
                    
                    {#if currentProject.metadata?.technology_stack}
                        <div class="mb-4">
                            <h4 class="text-sm font-medium text-gray-400 mb-2">Technology Stack</h4>
                            <div class="flex flex-wrap gap-2">
                                {#each currentProject.metadata.technology_stack as tech}
                                    <span class="bg-purple-900 text-purple-100 px-2 py-1 rounded text-xs">
                                        {tech}
                                    </span>
                                {/each}
                            </div>
                        </div>
                    {/if}
                    
                    {#if currentProject.metadata?.team_members}
                        <div>
                            <h4 class="text-sm font-medium text-gray-400 mb-2">Team Members</h4>
                            <div class="flex flex-wrap gap-2">
                                {#each currentProject.metadata.team_members as member}
                                    <span class="bg-gray-600 text-gray-200 px-2 py-1 rounded text-xs">
                                        {member}
                                    </span>
                                {/each}
                            </div>
                        </div>
                    {/if}
                </div>
                
                <!-- Milestones -->
                <div class="bg-gray-700 rounded-lg p-6 mb-6">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-lg font-semibold text-white">Milestones</h3>
                        
                        <div class="flex items-center space-x-2">
                            <input
                                type="text"
                                bind:value={newMilestoneTitle}
                                placeholder="New milestone title"
                                class="bg-gray-800 text-white placeholder-gray-400 border border-gray-600 rounded px-3 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                            />
                            <button
                                on:click={addMilestone}
                                disabled={!newMilestoneTitle.trim()}
                                class="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white p-1 rounded transition-colors"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                    
                    {#if currentProject.metadata?.milestones && currentProject.metadata.milestones.length > 0}
                        <div class="space-y-3">
                            {#each currentProject.metadata.milestones as milestone}
                                <div class="bg-gray-800 p-4 rounded-lg flex items-start">
                                    <div class="mr-3 mt-1">
                                        <button
                                            on:click={() => toggleMilestoneStatus(milestone.id)}
                                            class="w-5 h-5 rounded-full border {milestone.completed ? 'bg-green-600 border-green-600' : 'border-gray-500'} flex items-center justify-center"
                                        >
                                            {#if milestone.completed}
                                                <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path>
                                                </svg>
                                            {/if}
                                        </button>
                                    </div>
                                    
                                    <div class="flex-1">
                                        <h4 class="text-white font-medium {milestone.completed ? 'line-through text-gray-400' : ''}">{milestone.title}</h4>
                                        
                                        {#if milestone.description}
                                            <p class="text-sm text-gray-400 mt-1">{milestone.description}</p>
                                        {/if}
                                        
                                        <div class="flex items-center mt-2 text-xs text-gray-500">
                                            {#if milestone.due_date}
                                                <span>Due: {new Date(milestone.due_date).toLocaleDateString()}</span>
                                            {/if}
                                            
                                            {#if milestone.completed && milestone.completed_at}
                                                <span class="ml-3">Completed: {new Date(milestone.completed_at).toLocaleDateString()}</span>
                                            {/if}
                                        </div>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    {:else}
                        <div class="text-center py-6 text-gray-400">
                            <p>No milestones yet. Add your first milestone to get started.</p>
                        </div>
                    {/if}
                </div>
                
                <!-- Repository Stats -->
                {#if currentProject.metadata?.repository_stats}
                    <div class="bg-gray-700 rounded-lg p-6">
                        <h3 class="text-lg font-semibold text-white mb-4">Repository Statistics</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                            <div class="bg-gray-800 p-4 rounded-lg">
                                <h4 class="text-sm font-medium text-gray-400 mb-1">Total Commits</h4>
                                <p class="text-white font-semibold">{currentProject.metadata.repository_stats.total_commits}</p>
                            </div>
                            
                            <div class="bg-gray-800 p-4 rounded-lg">
                                <h4 class="text-sm font-medium text-gray-400 mb-1">Total Lines</h4>
                                <p class="text-white font-semibold">{currentProject.metadata.repository_stats.total_lines.toLocaleString()}</p>
                            </div>
                            
                            <div class="bg-gray-800 p-4 rounded-lg">
                                <h4 class="text-sm font-medium text-gray-400 mb-1">Last Commit</h4>
                                <p class="text-white font-semibold">{new Date(currentProject.metadata.repository_stats.last_commit_date).toLocaleDateString()}</p>
                            </div>
                        </div>
                        
                        <div>
                            <h4 class="text-sm font-medium text-gray-400 mb-2">Languages</h4>
                            <div class="space-y-2">
                                {#each Object.entries(currentProject.metadata.repository_stats.languages) as [language, lines]}
                                    {@const lineCount = Number(lines)}
                                    <div>
                                        <div class="flex justify-between text-xs mb-1">
                                            <span class="text-white">{language}</span>
                                            <span class="text-gray-400">{lineCount.toLocaleString()} lines ({Math.round(lineCount / currentProject.metadata.repository_stats.total_lines * 100)}%)</span>
                                        </div>
                                        <div class="w-full bg-gray-800 rounded-full h-2">
                                            <div
                                                class="bg-purple-600 h-2 rounded-full"
                                                style="width: {Math.round(lineCount / currentProject.metadata.repository_stats.total_lines * 100)}%"
                                            ></div>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </div>
                    </div>
                {/if}
            </div>
        {/if}
    </div>
</div>
