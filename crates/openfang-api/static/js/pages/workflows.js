// OpenFang Workflows Page — Workflow builder + run history
'use strict';

function workflowsPage() {
  return {
    // -- Workflows state --
    workflows: [],
    showCreateModal: false,
    runModal: null,
    runInput: '',
    runResult: '',
    running: false,
    loading: true,
    loadError: '',
    newWf: { name: '', description: '', steps: [{ name: '', agent_name: '', mode: 'sequential', prompt: '{{input}}' }] },
    editModal: null,
    editWf: { name: '', description: '', steps: [] },

    // -- Workflows methods --
    async loadWorkflows() {
      this.loading = true;
      this.loadError = '';
      try {
        this.workflows = await OpenFangAPI.get('/api/workflows');
      } catch(e) {
        this.workflows = [];
        this.loadError = e.message || 'Could not load workflows.';
      }
      this.loading = false;
    },

    async loadData() { return this.loadWorkflows(); },

    async createWorkflow() {
      var steps = this.newWf.steps.map(function(s) {
        return { name: s.name || 'step', agent_name: s.agent_name, mode: s.mode, prompt: s.prompt || '{{input}}' };
      });
      try {
        var wfName = this.newWf.name;
        await OpenFangAPI.post('/api/workflows', { name: wfName, description: this.newWf.description, steps: steps });
        this.showCreateModal = false;
        this.newWf = { name: '', description: '', steps: [{ name: '', agent_name: '', mode: 'sequential', prompt: '{{input}}' }] };
        OpenFangToast.success('工作流 "' + wfName + '" 已创建');
        await this.loadWorkflows();
      } catch(e) {
        OpenFangToast.error('创建工作流失败: ' + e.message);
      }
    },

    showRunModal(wf) {
      this.runModal = wf;
      this.runInput = '';
      this.runResult = '';
    },

    async executeWorkflow() {
      if (!this.runModal) return;
      this.running = true;
      this.runResult = '';
      try {
        var res = await OpenFangAPI.post('/api/workflows/' + this.runModal.id + '/run', { input: this.runInput });
        this.runResult = res.output || JSON.stringify(res, null, 2);
        OpenFangToast.success('工作流已完成');
      } catch(e) {
        this.runResult = 'Error: ' + e.message;
        OpenFangToast.error('工作流失败: ' + e.message);
      }
      this.running = false;
    },

    async viewRuns(wf) {
      try {
        var runs = await OpenFangAPI.get('/api/workflows/' + wf.id + '/runs');
        this.runResult = JSON.stringify(runs, null, 2);
        this.runModal = wf;
      } catch(e) {
        OpenFangToast.error('加载运行历史失败: ' + e.message);
      }
    },

    async deleteWorkflow(wf) {
      if (!confirm('删除工作流 "' + wf.name + '"? 此操作不可撤销。')) return;
      try {
        await OpenFangAPI.delete('/api/workflows/' + wf.id);
        OpenFangToast.success('工作流 "' + wf.name + '" 已删除');
        await this.loadWorkflows();
      } catch(e) {
        OpenFangToast.error('删除工作流失败: ' + e.message);
      }
    },

    async showEditModal(wf) {
      try {
        var full = await OpenFangAPI.get('/api/workflows/' + wf.id);
        this.editWf = {
          name: full.name || '',
          description: full.description || '',
          steps: (full.steps || []).map(function(s) {
            return {
              name: s.name || '',
              agent_name: (s.agent && s.agent.name) || '',
              mode: s.mode || 'sequential',
              prompt: s.prompt_template || '{{input}}'
            };
          })
        };
        if (this.editWf.steps.length === 0) {
          this.editWf.steps.push({ name: '', agent_name: '', mode: 'sequential', prompt: '{{input}}' });
        }
        this.editModal = wf;
      } catch(e) {
        OpenFangToast.error('加载工作流失败: ' + e.message);
      }
    },

    async saveWorkflow() {
      if (!this.editModal) return;
      var steps = this.editWf.steps.map(function(s) {
        return { name: s.name || 'step', agent_name: s.agent_name, mode: s.mode, prompt: s.prompt || '{{input}}' };
      });
      try {
        var wfName = this.editWf.name;
        await OpenFangAPI.put('/api/workflows/' + this.editModal.id, { name: wfName, description: this.editWf.description, steps: steps });
        this.editModal = null;
        OpenFangToast.success('工作流 "' + wfName + '" 已更新');
        await this.loadWorkflows();
      } catch(e) {
        OpenFangToast.error('更新工作流失败: ' + e.message);
      }
    }
  };
}
